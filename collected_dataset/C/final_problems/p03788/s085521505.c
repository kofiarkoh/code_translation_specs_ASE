#include<stdio.h>

char s[200001];

int main(int argc, char const *argv[]){
	long n,k,d,f=0,i;
	char p,pp;
	scanf("%ld %ld",&n,&k);
	getchar();
	scanf("%s",s);
	d=n-1;
	while(k>0){
		if(s[0]=='B'){
			p=s[d+1];
			for(i=d;i>=0;i--){
				pp=s[i];
				if(p=='A') s[i]='B';
				else s[i]='A';
				p=pp;
			}
			while(d>0 && s[d-1]!=s[d]){
				d--;
			}
		}else s[0]='B';
		k--;
		if(d==0){
			f=1;
			break;
		}
	}
	if(f==1){
		if(s[1]!='A'){
			if(k%2==1){
				s[0]=(s[0]=='B')?'A':'B';
			}
		}
		printf("%s\n",s);
	}else printf("%s\n",s);
	return 0;
}