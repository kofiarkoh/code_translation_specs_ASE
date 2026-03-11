#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#define ll long long int
#define lim 100010
#define MIN(x,y) ((x)<(y)?(x):(y))
#define MAX(x,y) ((x)<(y)?(y):(x))
#define ABS(x) ((x)>0?(x):-(x))

int main(void){
	int n;
	scanf("%d",&n);
	char S[n][11];
	for(int i=0;i<n;i++)scanf("%s",S[i]);
	int moto=0;
	int headB=0,tailA=0,ryouhou=0;
	bool a=false,b=false;
	for(int i=0;i<n;i++){
		a = b = false;
		int l = strlen(S[i]);
		if(S[i][0]=='B'){
			headB++;
			b=true;
		}
		if(S[i][l-1]=='A'){
			tailA++;
			a=true;
		}
		if(a&&b)ryouhou++;
		for(int j=0;j<l-1;j++){
			if(S[i][j]=='A' && S[i][j+1]=='B')moto++;
		}
	}
	int tmp = MIN(headB,tailA);
	if(MAX(headB,tailA)<=ryouhou)tmp--;
	int ans=moto+tmp;
	printf("%d",ans);
	return 0;
}