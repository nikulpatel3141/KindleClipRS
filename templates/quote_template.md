{% macro format_loc(page_, location_) -%}
{% match page_ -%}
  {% when Some with (x) -%}
    (page: {{ x }})
  {%- when None -%}
  {% match location_ -%}
    {% when Some with (x) -%}
      (location: {{ x.0 }})
    {%- when None %}
  {%- endmatch %}
{%- endmatch %}
{%- endmacro %}

{%- if true -%} {# clear newlines on ends -#}
- *{{ clipping_type }}* {% call format_loc(page, location) %}
`{{ quote }}`
{%- endif -%}