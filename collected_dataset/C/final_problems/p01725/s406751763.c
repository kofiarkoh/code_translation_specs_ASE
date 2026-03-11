#include <stdio.h>
#include <inttypes.h>
#include <ctype.h>

int tokenNum;
int64_t tokens[256];

int yusen[8];

int exprNum;
int64_t expr[256];

void gotiusa(void) {
	int i;
	int stackNum = 0;
	int64_t stack[256];
	exprNum = 0;
	for (i = 0; i < tokenNum; i++) {
		switch (tokens[i]) {
			case -1: case -2: case -3:
				while (stackNum > 0 && yusen[-stack[stackNum - 1]] >= yusen[-tokens[i]]) {
					expr[exprNum++] = stack[--stackNum];
				}
				stack[stackNum++] = tokens[i];
				break;
			case -4:
				stack[stackNum++] = -4;
				break;
			case -5:
				while (stackNum > 0 && stack[stackNum - 1] != -4) {
					expr[exprNum++] = stack[--stackNum];
				}
				if (stackNum > 0) stackNum--;
				break;
			default:
				expr[exprNum++] = tokens[i];
				break;
		}
	}
	while (stackNum > 0) {
		expr[exprNum++] = stack[--stackNum];
	}
}

int64_t kinnmoza(void) {
	int i;
	int stackNum = 0;
	int64_t stack[256];
	for (i = 0; i < exprNum; i++) {
		switch (expr[i]) {
			case -1:
				if (stackNum >= 2) {
					stack[stackNum - 2] += stack[stackNum - 1];
					stackNum--;
				}
				break;
			case -2:
				if (stackNum >= 2) {
					stack[stackNum - 2] -= stack[stackNum - 1];
					stackNum--;
				}
				break;
			case -3:
				if (stackNum >= 2) {
					stack[stackNum - 2] *= stack[stackNum - 1];
					stackNum--;
				}
				break;
			default:
				if (expr[i] >= 0) stack[stackNum++] = expr[i];
				break;
		}
	}
	return stackNum > 0 ? stack[stackNum - 1] : 0;
}

int main(void) {
	char asumikana[256];
	int i;
	int64_t mizukinana = INT64_MIN;
	if (scanf("%255s", asumikana) != 1) return 1;
	for (i = 0; asumikana[i] != '\0'; i++) {
		switch (asumikana[i]) {
			case '+': tokens[tokenNum++] = -1; break;
			case '-': tokens[tokenNum++] = -2; break;
			case '*': tokens[tokenNum++] = -3; break;
			case '(': tokens[tokenNum++] = -4; break;
			case ')': tokens[tokenNum++] = -5; break;
			default:
				if (isdigit((unsigned char)asumikana[i])) {
					int64_t num = 0;
					while (isdigit((unsigned char)asumikana[i])) {
						num = num * 10 + (asumikana[i] - '0');
						i++;
					}
					i--;
					tokens[tokenNum++] = num;
				} else {
					puts("INVALID TOKEN");
					return 2;
				}
				break;
		}
	}
	for (yusen[1] = 1; yusen[1] <= 3; yusen[1]++) {
		
		for (yusen[2] = 1; yusen[2] <= 3; yusen[2]++) {
			for (yusen[3] = 1; yusen[3] <= 3; yusen[3]++) {
				int64_t mizuhasu;
				gotiusa();
				mizuhasu = kinnmoza();
				if (mizuhasu > mizukinana) mizukinana = mizuhasu;
			}
		}
	}
	printf("%" PRId64 "\n", mizukinana);
	return 0;
}

