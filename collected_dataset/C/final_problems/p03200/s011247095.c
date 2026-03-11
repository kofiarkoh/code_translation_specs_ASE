#include<stdio.h>

int main(){
	char s[200000];scanf("%s",s);
	long ans=0;
	int i,n=0,f;while(s[n++]!=0)
	while(1){
		f=0;
		for(i=0;i<n-1;i++){
			if(s[i]=='B'&&s[i+1]=='W'){
				s[i]='W';
				s[i+1]='B';
				ans++;
				f=1;
			}
		}
		if(f==0)break;
	}
	printf("%ld\n",ans);
	return 0;
}