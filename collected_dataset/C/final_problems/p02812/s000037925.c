#include<stdio.h>
#include<string.h>
int main()
{
	int N,i,count=0;
	char S[50];
	scanf("%d %s", &N,&S);
	for (i = 0; i < N; i++) {
		if (S[i] == 'A') {
			if (S[i + 1] == 'B') {
				if (S[i + 2] == 'C') {
					count++;
				}
			}
		}
	}
	printf("%d", count);
    return 0;
}

