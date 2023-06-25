{% macro format_loc(page_, location_) -%} {# FIXME: repetition -#}
{% match page_ -%}
  {% when Some with (x) -%}
    (page: {{ x }})
  {%- when None -%}
  {% match location_ -%}
    {% when Some with (x) -%}
      (location: {{ x }})
    {%- when None %}
  {%- endmatch %}
{%- endmatch %}
{%- endmacro %}

{%- if true -%} {# clear newlines on ends -#}
- *{{ clipping_type }}* {% call format_loc(page, location) %}
  > {{ quote }}

{%~ endif %}
---