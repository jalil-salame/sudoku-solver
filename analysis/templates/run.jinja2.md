{% from "benches.jinja2.md" import render %}
{% block changes %}{% endblock changes %}
### Benchmark results
{% for bench in benches %}
- {{ bench.name }}
{%- endfor %}
{% for bench in benches -%}
{{ render(bench, run) }}
{% endfor %}
