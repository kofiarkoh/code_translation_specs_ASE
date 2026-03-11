#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<math.h>

//for
#define REP(i,n) for(int i=0,i##_len=(int)(n);i<i##_len;i++)
#define REPS(i,n) for(int i=1,i##_len=(int)(n);i<=i##_len;i++)
#define RREP(i,n) for(int i=(int)(n)-1;i>=0;i--)
#define RREPS(i,n) for(int i=(int)(n);i>0;i--)

//printf
#define PRINTD(d) printf("%d\n",(d))
#define PRINTS(s) printf("%s\n",(s))

//memset
#define m0(s) memset(s,0,sizeof(s))
#define ml(s) memset(s,63,sizeof(s))
#define fill(s,c) memset(s,c,sizeof(s))

typedef long long ll;
int Min(int a, int b) {
	return (a)<(b) ? (a) : (b);
}
int Max(int a, int b) {
	return (a)>(b) ? (a) : (b);
}

int main() {
	char S[100000];
	scanf("%s", S);
	int len = strlen(S);
	int l=len/2-1, r=(len+1)/2;
	char p = S[(len - 1) / 2];
	int cnt = 0;
	REP(i, len/2-1) {
		if (S[l--] != p||S[r++] != p) {
			break;
		}
		cnt++;
	}
	PRINTD((len+1)/2+cnt);
}