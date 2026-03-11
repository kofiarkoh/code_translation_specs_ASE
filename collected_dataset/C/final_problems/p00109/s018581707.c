#include <stdio.h>
#include <stdlib.h>

enum command_type {
	VALUE = 0x2,
	OP_ADD = 0x4,
	OP_SUB = 0x5,
	OP_MUL = 0x8,
	OP_DIV = 0x9,
	END = 0xffff
};
typedef enum command_type CommandType;

struct command_t {
	CommandType type;
	int value;
};
typedef struct command_t Command;

void expression(void);

char g_tokens[128][128];
char *g_symbol;
int g_index;
Command g_commands[128];
int g_cmd_idx = 0;

void command_append(CommandType type, int value) {
	Command *new;

	new = &g_commands[g_cmd_idx];
	new->type = type;
	new->value = value;
	g_cmd_idx++;
}

void next_symbol(void) {
	g_symbol = g_tokens[g_index++];
}

void factor(void) {
	int value;

	if ('0' <= g_symbol[0] && g_symbol[0] <= '9') {
		value = atoi(g_symbol);
		command_append(VALUE, value);
		next_symbol();
	} else if (g_symbol[0] == '(') {
		next_symbol();
		expression();
		if (g_symbol[0] != ')') {
			fprintf(stderr, "SyntaxError: close %s\n", g_symbol);
			exit(1);
		}
		next_symbol();
	} else {
		fprintf(stderr, "SyntaxError %s\n", g_symbol);
		exit(1);
	}
}

void term(void) {
	char op_sign;

	factor();
	while (g_symbol[0] == '*' || g_symbol[0] == '/') {
		op_sign = g_symbol[0];
		next_symbol();
		factor();
		command_append(op_sign == '*' ? OP_MUL : OP_DIV, 0);
	}
}

void expression(void) {
	char op_sign;

	if (g_symbol[0] == '+' || g_symbol[0] == '-') {
		op_sign = g_symbol[0];
		next_symbol();
		term();
		command_append(op_sign == '+' ? OP_ADD : OP_SUB, 0);
	} else {
		term();
	}
	while (g_symbol[0] == '+' || g_symbol[0] == '-') {
		op_sign = g_symbol[0];
		next_symbol();
		term();
		command_append(op_sign == '+' ? OP_ADD : OP_SUB, 0);
	}
}

void parse(void) {
	g_index = 0;
	g_cmd_idx = 0;

	next_symbol();
	while (g_symbol[0] != '=') {
		expression();
	}
	command_append(END, 0);
}


void tokenize(char *in) {
	int digit, n, i;

	digit = n = i = 0;
	for (; *in; in++) {
		if ('0' <= *in && *in <= '9') {
			g_tokens[n][i++] = *in;
			digit = 1;
		} else {
			if (digit) {
				n++;
				digit = 0;
				i = 0;
			}
			g_tokens[n++][0] = *in;
			if (*in == '=')
				return;
		}
	}
}

int run(void) {
	int stack[128];
	int top = -1, buf;
	Command *cmd;
	int i;

	for (i = 0; g_commands[i].type != END; i++) {
		cmd = &g_commands[i];
		//for (cmd = g_commands; cmd != NULL; cmd = cmd->next) {
		if (cmd->type == VALUE) {
			stack[++top] = cmd->value;
		} else if (cmd->type == OP_ADD) {
			buf = stack[top--];
			stack[top] += buf;
		} else if (cmd->type == OP_SUB) {
			buf = stack[top--];
			stack[top] -= buf;
		} else if (cmd->type == OP_MUL) {
			buf = stack[top--];
			stack[top] *= buf;
		} else if (cmd->type == OP_DIV) {
			buf = stack[top--];
			stack[top] /= buf;
		}
	}
	return stack[top];
}

int main(void) {
	int n, i;
	char in[128];

	scanf("%d ", &n);

	while (n--) {
		for (i = 0; i < 128; i++) {
			g_tokens[i][0] = '\0';
		}
		fgets(in, 127, stdin);
		tokenize(in);
		parse();
		printf("%d\n", run());
	}
	return 0;
}