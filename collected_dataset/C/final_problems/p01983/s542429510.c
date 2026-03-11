// AOJ 2883: Proof of Knowledge
// 2018.7.11 bal4u

#include <stdio.h>

char tr['d'+1];

char *calc(int *ans, char *S, char *P)
{
	int v1, v2;
	char op;
	
    if (*S == '[') {
        op = *(++S), S++;
        if (*S == '[') S = calc(&v1, S, P);
        else v1 = P[tr[*S++]];

        if (*S == '[') S = calc(&v2, S, P);
        else v2 = P[tr[*S++]];
		
        if      (op == '+') *ans = v1 | v2;
        else if (op == '*') *ans = v1 & v2;
        else                *ans = v1 ^ v2;
	} else *ans = P[tr[*S++]];
    while (*S == ']') S++;
    return S;
}

char S[100];
char p[10000][4];

int main()
{
	int i, k, val, cnt;
	char P[10];

	for (i = 0; i < 4; i++) tr['a'+i] = i;
	for (i = 0; i < 10000; i++) {
		p[i][0] = i/1000;
		p[i][1] = (i/100)%10;
		p[i][2] = (i/10)%10;
		p[i][3] = i%10;
	}

	while (fgets(S, 100, stdin) && *S != '.') {
		fgets(P, 10, stdin);
		for (i = 0; i < 4; i++) P[i] &= 0xf;
		calc(&val, S, P), cnt = 0;
		for (i = 0; i < 10000; i++) {
			calc(&k, S, p[i]);
			if (k == val) cnt++;
		}
		printf("%d %d\n", val, cnt);
	}
	return 0;
}

