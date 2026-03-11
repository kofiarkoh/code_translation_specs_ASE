#include <stdio.h>
#include <string.h>

char S[105];

int main() {

	int N;

	scanf("%d",&N);

	scanf("%s",S);

	int len=strlen(S);

	int i,j;

	int ans=0;

	for(i=0;i<len;i++){
		int history[2][26]={0};
		for(j=0;j<=i;j++){
		history[0][S[j]-'a']++;
		}
		for(j=i+1;j<len;j++){
		history[1][S[j]-'a']++;
		}
		int cnt=0;

		for(j=0;j<26;j++){
			if(history[0][j]>0&&history[1][j]>0){cnt++;}
		}

		if(ans<cnt){ans=cnt;}
	}

	printf("%d\n",ans);

	return 0;
}