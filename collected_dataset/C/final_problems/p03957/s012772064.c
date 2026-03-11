
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>
#include <math.h>

#define MOD 1000000007
#define REP(i,n) for ((i)=0;i<(n);i++)
#define SET(a,c) memset(a,c,sizeof a)
#define CLR(a) memset(a,0,sizeof a)
#define MIN(a,b) (a>b?b:a)
#define MAX(a,b) (a>b?a:b)
#define LL long long
#define ULL unsigned long long

int compare_ll(const void *a, const void *b){
	return *(ULL*)b - *(ULL*)a;
}



int main(void) {

	char s[101];
	int flag = 0;
	int i = 0;

	scanf("%s", &s);

	while(s[i]!='\0'){
		if(s[i] == 'C'){
			while(s[i]!='\0'){
				if(s[i]== 'F'){
					flag = 1;
					break;
				}else{
					i++;
				}
			}
		}else{
			i++;
		}
	}

	if(flag == 1){
		puts("Yes");
	}else{
		puts("No");
	}

	return 0;
}

/* 最後の改行忘れるな！！ */
