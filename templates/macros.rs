{% macro nop(op) %}
{% endmacro %}

{% macro ld(op) %}
    let res = {{ op.operand[1] | getter(bits=op.bits) }}
    {{ op.operand[0] | setter(bits=op.bits) }}res);
{% endmacro %}



