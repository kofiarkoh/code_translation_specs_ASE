#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
 
#define INF 1000000000
 
#define min(a,b) (((a)<(b))?(a):(b))
#define max(a,b) (((a)>(b))?(a):(b))

int main() {
	long N;
	char S[1001];
	long i, j;
	int len;
	int f[4] = {0};
	
	scanf("%s", S);
	len = strlen(S);
	
	for(i=0;i<len;i++) {
		if(S[i] == 'N') f[0] = 1;
		if(S[i] == 'W') f[1] = 1;
		if(S[i] == 'S') f[2] = 1;
		if(S[i] == 'E') f[3] = 1;
	}
	
	if(f[0] - f[2] == 0 && f[1] - f[3] == 0) {
		printf("Yes\n");
	} else {
		printf("No\n");
	}
	
	return 0;
	
}