#include<stdio.h>
int main(){
	int a,b,n;
	int p[80000];
	p[0]=2;
	n=1;
	for(a=3;a<60000;a++){
		for(b=0;b<n;b++){
			if(a%p[b]==0){break;}
			if(a/p[b]<p[b]){
				p[n]=a;
				n++;
				break;
			}
		}
	}
	p[n]=60000;
	for(;;){
		if(scanf("%d",&a)==EOF){break;}
		n=0;
		while(a>=p[n]){
			n++;
		}
		if(p[n-1] == a){printf("%d %d\n",p[n-2],p[n]);}
		else{printf("%d %d\n",p[n-1],p[n]);}
	}
	return 0;
}