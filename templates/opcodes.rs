{% import "macros.rs" as macros  %}

{% for op in opcodes %}
    {{ op.operands[0] | getter(bits=op.bits) }}
{% endfor %}
