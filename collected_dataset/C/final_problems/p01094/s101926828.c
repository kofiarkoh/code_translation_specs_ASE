#define _CRT_SECURE_NO_WARNINGS
//#define _USE_MATH_DEFINES
#include<stdio.h>
//#include<stdlib.h>
//#include<math.h>
#include<string.h>
//#include<time.h>
//#define pi acos(-1.0)
#define P(type,x) fprintf(stdout,"%"#type"\n",x)
#define S(type,x) fscanf(stdin,"%"#type,&x)
int main() {
	int vote[26],n;
	char a,tmp;
	while (S(d, n), n) {
		getchar();
		int max, sec,i;
		max = sec = i = 0;
		tmp = 'A';
		memset(vote, 0, sizeof(vote));
		while (n--) {
			fscanf(stdin, "%c%*c", &a);
			if (max > sec + n+1)continue;
			i++;
			vote[a - 'A']++;
			if (vote[a - 'A'] >= max) {
				if(vote[tmp-'A']-1!=max) sec = max;
				max = vote[a - 'A'], tmp = a;
			}
			else if (vote[a - 'A'] > sec) sec = vote[a - 'A'];
		}
		if (max == sec) puts("TIE");
		else fprintf(stdout, "%c %d\n", tmp, i);
	}
	return 0;
}