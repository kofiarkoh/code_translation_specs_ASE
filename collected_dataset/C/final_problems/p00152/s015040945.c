#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int i,j,n,score[50][2],order[50],frame,first,last,pin,str,spa;

void swap (int* x,int* y){
	int z;
	z=*x;
	*x=*y;
	*y=z;
}

int main(){
	while(scanf("%d",&n)*n){
		for(i=0;i<n;i++){
			scanf("%d",&score[i][0]);

			frame=1;score[i][1]=0;last=2;first=-1;
			while(frame<10+last){
				scanf("%d",&pin);
				score[i][1]+=pin*(1+(str+1)/2+spa);

				if(str>0)str-=(str+1)/2;
				spa=0;
				if(frame>9)frame++;
				if(pin==10){
					if(frame<10){str+=2;if(frame<10)frame++;}
					else last=3;
				}
				else{
					if(first<0)first=pin;
					else{
						if(first+pin==10){
							if(frame<10)spa=1;
							else last=3;
						}
						first=-1;
						if(frame<10)frame++;
					}
				}
			}

			order[i]=i;
			for(j=i;j>0;j--){
				if(score[i][1]>score[order[j-1]][1] || (score[i][1]==score[order[j-1]][1] && score[i][0]<score[order[j-1]][0])){
					swap(&order[j],&order[j-1]);
				}
			}
		}
		for(i=0;i<n;i++)printf("%d %d\n",score[order[i]][0],score[order[i]][1]);
	}
	return 0;
}