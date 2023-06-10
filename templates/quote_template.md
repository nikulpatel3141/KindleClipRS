{% match location %}
{% when Some(x) %} Somehere
{% when None %} Nonehere
{% endmatch %}

- {{ quote }}