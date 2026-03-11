#include <stdio.h>

int main(void)
{
	int m[100],f[100],r[100];
	char ans[100];
	int i=0,j,sum=0,a;
	
	while(1){
		scanf("%d %d %d",&m[i],&f[i],&r[i]);
		if(m[i]==-1 && f[i]==-1 && r[i]==-1) break;
		else i++;
	}
	a=i;
	for(j=0 ; j<i ; j++){
		sum=m[j]+f[j];
		if(m[j]==-1 || f[j]==-1){
			ans[j]='F';
		}
		else if(sum>=80){
			ans[j]='A';
		}
		else if(sum>=65){
			ans[j]='B';
		}
		else if(sum>=50){
			ans[j]='C';
		}
		else if(sum>=30){
			if(r[j]>=50)
				ans[j]='C';
			else ans[j]='D';
		}
		else ans[j]='F';
		sum=0;
	}
	
	for (i=0 ; i<a ; i++)
		printf("%c\n",ans[i]);
	
	return 0;
}
