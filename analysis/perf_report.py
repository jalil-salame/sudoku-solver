import math
import os
import re
import shutil
import subprocess
from dataclasses import dataclass
from os.path import isabs
from pathlib import Path
from tempfile import TemporaryDirectory
from typing import Any, Self

import click
import matplotlib.pyplot as plt
import seaborn as sns
import toml
from jinja2 import Environment, FileSystemLoader, select_autoescape
from pandas import DataFrame

RUN: bool = False
CONFIG: dict[str, Any] = {}
OUTDIR: Path = Path("analysis").resolve()
ASSETDIR: Path = OUTDIR / "assets"
DATADIR: Path = OUTDIR / "data"
TMPL_DIR: Path = OUTDIR / "templates"
RUN_TEMPLATE: Path = TMPL_DIR / "run_template.jinja2.md"


def assert_type_of[T](var: str, value: Any, typ: type[T]) -> T:
    if not isinstance(value, typ):
        raise TypeError(
            f"expected {var} to be a {typ} but it was a {type(value)} instead"
        )
    return value


# LL (Last Level (usually L3 in modern computers))
@dataclass
class IaiData:
    instr_count: int
    l1_instr_miss: int
    ll_instr_miss: int
    data_reads: int
    l1_data_read_miss: int
    ll_data_read_miss: int
    data_writes: int
    l1_data_write_miss: int
    ll_data_write_miss: int

    @classmethod
    def from_file(cls, data: Path) -> Self:
        with data.open("rt") as fp:
            events = None
            summary = None
            for line in fp:
                if line.startswith("events: "):
                    events = line.lstrip("events: ")
                if line.startswith("summary: "):
                    summary = line.lstrip("summary: ")
            if events is None or summary is None:
                not_found = "events" if events is None else "summary"
                raise ValueError(f"didn't find {not_found} in {data.as_posix()}")
            events = events.strip().split(" ")
            summary = [int(val) for val in summary.strip().split(" ")]
            event_count = dict(zip(events, summary, strict=True))
            return cls(
                event_count["Ir"],
                event_count["I1mr"],
                event_count["ILmr"],
                event_count["Dr"],
                event_count["D1mr"],
                event_count["DLmr"],
                event_count["Dw"],
                event_count["D1mw"],
                event_count["DLmw"],
            )


@dataclass
class Bench:
    name: str
    shell: bool
    command: list[str]
    output: str

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Self:
        name = assert_type_of("name", data["name"], str)
        shell = (
            assert_type_of("shell", data["shell"], bool) if "shell" in data else False
        )
        command = assert_type_of("command", data["command"], list)
        output = assert_type_of("output", data["output"], str)
        return cls(name, shell, command, output)


@dataclass
class Run:
    rev: str

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Self:
        rev = assert_type_of("rev", data["rev"], str)
        return cls(rev)


RUNS: list[Run] = []
BENCHES: list[Bench] = []


def get_runs() -> list[Run]:
    runs = assert_type_of("runs", CONFIG["runs"], list)
    return [Run.from_dict(run) for run in runs]


def get_benches() -> list[Bench]:
    benches = assert_type_of("benches", CONFIG["benches"], list)
    return [Bench.from_dict(bench) for bench in benches]


def find_repo() -> Path:
    cwd = Path.cwd()
    if "GIT_DIR" in os.environ:
        return Path(os.environ["GIT_DIR"])
    elif (cwd / ".git").exists():
        return cwd
    raise ValueError("failed to autodetect git repo, please specify it manually")


def clone_repo(repo: str) -> TemporaryDirectory[str]:
    dir = TemporaryDirectory(prefix="sudoku-checkout.git.")
    print(f"[INFO]: Created tempdir for repo {dir}")
    if RUN:
        subprocess.run(["git", "clone", "--bare", repo, dir.name], check=True)
    print(f"[INFO]: Cloned {repo} to {dir}")
    return dir


def checkout_rev(repo: str, dir: str, rev: str):
    print(f"[INFO]: Checking out {rev}")
    if RUN:
        subprocess.run(["git", "restore", "."], cwd=dir)
        subprocess.run(
            [
                "git",
                f"--git-dir={repo}",
                f"--work-tree={dir}",
                "switch",
                "--detach",
                rev,
            ],
            check=True,
        )


def create_run_template(pad: int, ix: int, run: Run) -> None:
    path = TMPL_DIR / f"{ix:0{pad}}-{run.rev}.jinja2.md"
    if path.exists():
        print(f"[INFO]: Found existing {path.as_posix()}, skipping template creation")
        return
    for file in TMPL_DIR.iterdir():
        if run.rev in file.name:
            print(
                f"[INFO]: Found existing {file.as_posix()}, skipping template creation"
            )
            if file == path:
                return
            print(f"[INFO]: Renaming {file.as_posix()} to {path.as_posix()}")
            if RUN:
                shutil.move(file, path)
            return
    print(f"[INFO]: Creating {path.as_posix()} from {RUN_TEMPLATE.as_posix()}")
    if RUN:
        shutil.copy(RUN_TEMPLATE, path)


def run_benches(checkout: str, run: Run, rerun: bool) -> None:
    CHECKOUT_PATH = Path(checkout)
    CHECKOUT_DATA = DATADIR / f"{run.rev}"
    if not CHECKOUT_DATA.exists():
        CHECKOUT_DATA.mkdir(parents=True)
    for bench in BENCHES:
        BENCH_DATA = CHECKOUT_DATA / bench.name
        if not rerun and BENCH_DATA.exists():
            print(f"[INFO]: ({run.rev}) Found previous run of {bench.name}, skipping")
            continue
        if not BENCH_DATA.exists():
            BENCH_DATA.mkdir()
        print(f"[INFO]: ({run.rev}) Running {bench.name}")
        command = list(map(os.path.expandvars, bench.command))
        print(f"[DEBUG]: Running `{" ".join(command)}`")
        if RUN:
            try:
                subprocess.run(command, cwd=CHECKOUT_PATH, check=True)
                print(f"[INFO]: ({run.rev}) Storing output of {bench.name}")
                output = Path(bench.output)
                if not isabs(output):
                    output = CHECKOUT_PATH / output
                if output.is_dir():
                    shutil.copytree(output, BENCH_DATA, dirs_exist_ok=True)
                else:
                    shutil.copy2(output, BENCH_DATA)
            except subprocess.CalledProcessError as e:
                print(f"[WARN]: Failed to run bench {bench.name}: {e}")
                BENCH_DATA.rmdir()
                print("\n".join(map(str, CHECKOUT_PATH.iterdir())))


def collapse_cr(line: bytes) -> str:
    ix = -1
    try:
        ix = line.rindex(b"\r")
    except ValueError:
        pass
    return line[ix + 1 :].decode("utf-8")


def insert_file(rev: str, filename: str) -> str:
    path = DATADIR / rev / filename
    if not path.exists():
        print(f"[ERROR]: {path.as_posix()} does not exist")
        return ""
    with path.open("rb") as fp:
        data = fp.readlines()
        data = [collapse_cr(line) for line in data]
        return "".join(data)


def regex_replace(data: str, regex: str, subs: str) -> str:
    reg = re.compile(regex, flags=re.MULTILINE)
    return reg.sub(subs, data)


def generate_report(templates: list[str]) -> None:
    env = Environment(
        loader=FileSystemLoader(TMPL_DIR),
        autoescape=select_autoescape(),
    )
    tmpls = [env.get_template(tmpl) for tmpl in templates]
    env.filters.update(regex_replace=regex_replace)
    env.globals.update(
        insert_file=insert_file,
        runs=list(reversed(list(zip(RUNS, tmpls)))),
        benches=BENCHES,
    )
    template = env.get_template("perflog.jinja2.md")
    rendered = template.render()
    with open("perflog.md", "wt") as fp:
        print(rendered, file=fp)


def set_globals(run: bool, config: Path) -> None:
    global RUN
    RUN = run
    global CONFIG
    with config.open("rt") as fp:
        CONFIG = toml.load(fp)
    global RUNS
    RUNS = get_runs()
    global BENCHES
    BENCHES = get_benches()


def gather_iai_data() -> DataFrame:
    res = dict()
    for run in RUNS:
        iai_data = DATADIR / run.rev / "iai" / "callgrind.solve_sudoku.first.out"
        if not iai_data.exists():
            print(f"[WARN]: couldn't find {iai_data.as_posix()}")
            continue
        data = IaiData.from_file(iai_data)
        res.setdefault("revision", []).append(run.rev)
        res.setdefault("instr count", []).append(data.instr_count)
        res.setdefault("instr L1 miss", []).append(data.l1_instr_miss)
        res.setdefault("instr LL miss", []).append(data.ll_instr_miss)
        res.setdefault("data fetched", []).append(data.data_reads)
        res.setdefault("L1 fetch miss", []).append(data.l1_data_read_miss)
        res.setdefault("LL fetch miss", []).append(data.ll_data_read_miss)
        res.setdefault("data written", []).append(data.data_writes)
        res.setdefault("L1 write miss", []).append(data.l1_data_write_miss)
        res.setdefault("LL write miss", []).append(data.ll_data_write_miss)
    return DataFrame(res)


def generate_iai_plot(iai_data: DataFrame) -> None:
    sns.set_theme(context="notebook", style="whitegrid", rc={"figure.figsize": (9, 5)})
    ax = sns.lineplot(iai_data, x="revision", y="instr count", errorbar=None)
    ax.set_ylabel("Instruction count")
    ax.set_xlabel("Git commit")
    ax.set_title("Instruction count over time")
    plt.savefig(ASSETDIR / "iai_data_progression.svg")
    plt.close()


@click.command()
@click.option("--repo", type=str, default=None)
@click.option("--config", type=Path, default=OUTDIR / "config.toml")
@click.option("--run/--dry-run", default=False)
@click.option("--rerun-benches/--no-rerun-benches", default=False)
def main(repo: str | None, config: Path, run: bool, rerun_benches: bool) -> None:
    set_globals(run, config)
    repo = repo if repo is not None else find_repo().as_posix()
    clone = clone_repo(repo)
    pad = int(math.log10(len(RUNS))) + 1
    with clone as repo, TemporaryDirectory(prefix="bench.") as dir:
        for ix, run_ in enumerate(RUNS):
            create_run_template(pad, ix, run_)
            checkout_rev(repo, dir, run_.rev)
            run_benches(dir, run_, rerun_benches)
    generate_report(
        templates=[f"{ix:0{pad}}-{run.rev}.jinja2.md" for ix, run in enumerate(RUNS)]
    )
    generate_iai_plot(gather_iai_data())


if __name__ == "__main__":
    main()
