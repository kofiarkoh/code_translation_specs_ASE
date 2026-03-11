#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <math.h>

#define P 1000000007

int comp(const void *a, const void *b){return *(int*)a-*(int*)b;}

int d[2][101][4];
int main(void){
	char s[103];
	int n, k, i, j;
	scanf("%s%d", s+1, &k);
	n=strlen(s+1);
	d[0][1][0]=1;
	if(s[1]!='0') d[0][1][1]=s[1]-'0'-1;
	d[1][1][1]=1;
	for(i=2; i<=n; i++){
		d[0][i][0]=1;
		for(j=1; j<=k; j++){
			d[0][i][j]=d[0][i-1][j]+d[0][i-1][j-1]*9;
			if(s[i]!='0') d[0][i][j]+=d[1][i-1][j-1]*(s[i]-'0'-1)+d[1][i-1][j];
			if(s[i]=='0'){
				d[1][i][j]=d[1][i-1][j];
			}else{
				d[1][i][j]=d[1][i-1][j-1];
			}
		}
	}
	printf("%d\n", d[0][n][k]+d[1][n][k]);
}
