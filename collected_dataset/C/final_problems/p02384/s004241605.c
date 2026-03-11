#include <stdio.h>
void spin1(int a[]);
void spin2(int a[]);

int main(void){
	int d[6],q,u,f,i;
	for(i=0;i<6;i++){
		scanf("%d",&d[i]);
	}
	scanf("%d",&q);
	while(q--){
		scanf("%d %d",&u,&f);
		if(d[0]==u||d[1]==u||d[5]==u||d[4]==u){
			while(d[0]!=u)
				spin1(d);
			while(d[1]!=f)
				spin2(d);
		}
		else{
			while(d[0]!=u)
				spin2(d);
			while(d[1]!=f)
				spin1(d);
		}
		printf("%d\n",d[2]);
	}
	
	return 0;
}

void spin1(int a[]){
	int t=a[0];
	a[0]=a[4];
	a[4]=a[5];
	a[5]=a[1];
	a[1]=t;
}
void spin2(int a[]){
	int t=a[1];
	a[1]=a[2];
	a[2]=a[4];
	a[4]=a[3];
	a[3]=t;
}