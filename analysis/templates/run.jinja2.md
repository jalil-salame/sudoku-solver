{% from "benches.jinja2.md" import render %}
{% block changes %}{% endblock changes %}
### Benchmark results
{% for bench in benches %}
- [{{ bench.name }} results](#{{ bench.name | lower | replace(" ", "-") }}{% if outer_loop.index0 > 0 %}-{{ outer_loop.index0 }}{% endif %})
{%- endfor %}
{% for bench in benches -%}
{{ render(bench, run) }}
{% endfor %}
