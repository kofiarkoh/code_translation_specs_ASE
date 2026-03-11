#include<stdio.h>
int main(void){
	int h1,w1,t1,h2,w2,t2,h3,w3,t3;
	int i,j,count;

	while(1){
		scanf("%d %d",&h1,&w1);
		if(h1==0&&w1==0)break;
		count=0;
		t1=h1*h1+w1*w1;
		w3=0;
		h3=0;
		t3=150*150+150*150;
		for(i=1;i<=150;i++){
			w2=i;
			for(j=1;j<=150;j++){
				h2=j;
				t2=h2*h2+w2*w2;
				if(t2==t1){
					if(h1<h2){//2
						if(h2<w2){
							h3=h2;
							w3=w2;
							count++;
						}
					}
				}
				if(count>=1)break;
			}
			if(count>=1)break;
		}
		for(i=1;i<=150;i++){
			h2=i;
			if(count>=1)break;
			for(j=1;j<=150;j++){
				w2=j;
				t2=h2*h2+w2*w2;
				if(t1<t2){//1
					if(h2<w2){
						if(t2<t3){
							h3=h2;
							w3=w2;
							t3=h3*h3+w3*w3;
						}
					}
				}
			}
		}
		printf("%d %d\n",h3,w3);
	}
}