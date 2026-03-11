#include<stdio.h>

int sort(int x[],int l){
	int i,j,k;
	for(i=1;i<=l;i++){
		for(j=l;j>=i+1;j--){
			if(x[j-1]<x[j]){
				k=x[j];
				x[j]=x[j-1];
				x[j-1]=k;
			}
		}
	}
	return 0;
}

int main(void){
	char s[16];
	int n,k,d[32]={0},i,c=0,b=0,x;
	scanf("%d%d",&n,&k);
	for(i=1;i<=n;i++){
		scanf("%s",s);
		x=s[0]-64;
		d[x]++;
	}
	while(1){
		sort(d,26);
		for(i=1;i<=k;i++){
			if(d[i]){
				d[i]--;
			}
			else{
				b++;break;
			}
		}
		if(b){break;}
		c++;
	}
	printf("%d\n",c);
	return 0;
}