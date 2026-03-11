#include <stdio.h>
int print_pan();

int p[5005]={};

int main(void) {
	int i,N,c,f;
	scanf("%d",&N);
	for(i=0;i<N;i++){
		scanf("%d",&p[i]);
	}
//	print_pan(N);
	c=0;
	f=0;
	while(f==0){
	for(i=1;i<N-1;i++){
		if(p[i]!=0){
			if(p[i-1]>p[i+1]){
				c=c+2;
				p[i-1]--;
				p[i]--;
			}else{
				c=c+2;
				p[i+1]--;
				p[i]--;
			}
		}
	}
	f=1;
	for(i=1;i<N-1;i++){if(p[i]>0){f=0;break;}}
	}
//	print_pan(N);
	if(p[0]>0){c=c+p[0];}
	if(p[N-1]>0){c=c+p[N-1];}
	printf("%d\n",c);

	return 0;
}
int print_pan(int N){
	int i;
	for(i=0;i<N-1;i++){printf("%d ",p[i]);}printf("%d\n",p[i]);
	return 0;
}

