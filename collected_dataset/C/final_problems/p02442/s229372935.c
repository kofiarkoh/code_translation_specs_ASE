#include<stdio.h>
#include<string.h>
#define N 1001

int main(){
	char a[N];
	char b[N];
	int x,n;
	
	scanf("%d",&n);
	for(int i=0;i<n;i++){
		scanf("%d",&x);
		a[i]=x+'0';
	}
	a[n]='\0';
	
	scanf("%d",&n);
	for(int i=0;i<n;i++){
		scanf("%d",&x);
		b[i]=x+'0';
	}
	b[n]='\0';
	
	if(strcmp(b,a)>0)printf("1\n");
	else printf("0\n");
	
	return 0;
	
}
