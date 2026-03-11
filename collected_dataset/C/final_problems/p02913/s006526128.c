#include <stdio.h> 
 
int 
main(){
	int N;
	char s[5010];
	scanf("%d",&N);scanf("%s",s);
	int i,j,k,len=0;
	int cnt = 0;
	for(i=0;i<=N-1-len;i++){
		for(j=i+len; j<=N-len;j++)
		{
			cnt = 0;
			for(k=0; i+k < j && j+k < N; k++){
				if(s[i+k]==s[j+k])cnt++;
				else break;				
			}
			if(len <cnt) len = cnt;
		}
	}
	printf("%d\n",len);
 return 0;
}