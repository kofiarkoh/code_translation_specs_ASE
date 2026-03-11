#include<stdio.h>

int main(void){
	int ball[3];
	int x,y,temp;
	char a,b;
	ball[0]=1;ball[1]=0;ball[2]=0;
	while(scanf("%c,%c",&a,&b)!=EOF){
		switch(a){
		case 'A':x=0;break;
		case 'B':x=1;break;
		case 'C':x=2;break;
		}
		switch(b){
		case 'A':y=0;break;
		case 'B':y=1;break;
		case 'C':y=2;break;
		}
		temp=ball[x];ball[x]=ball[y];ball[y]=temp;
	}
	if(ball[0]==1){
		printf("A\n");
	}else if(ball[1]==1){
		printf("B\n");
	}else{printf("C\n");}
	return 0;
}