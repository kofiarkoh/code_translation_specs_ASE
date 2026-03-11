#include<stdio.h>
#include<string.h>
int main(){
	char s[100];
	char p[100];
	int k=0, i, n, m, pl=0, sl=0;
	scanf("%s",s);
	scanf("%s",p);
	m=strlen(s);
	n=strlen(p);
	for(i=m;i<=99;i++){
		s[i]=s[k];
		k=k+1;
	}
	k=0;	
	m=strlen(s);
	while(pl<n){
		if(s[sl]==p[pl]){
			pl=pl+1;
		}else{
			pl=0;
		}
		sl=sl+1;
		if(sl>m){
			k=1;
			break;
		}
	}
	if(k==0){
		printf("Yes\n");
	}else if(k==1){
		printf("No\n");
	}
	return 0;
}