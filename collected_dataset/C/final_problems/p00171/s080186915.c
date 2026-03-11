#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#define GOOD 'a'-'A'

int pos[8],graph[50][50],used[8];
int SET;
char cube[8][8];

void roll(int* a,int *b){
	int c=7-*b;
	*b=*a;
	*a=c;
}

void swap (int* x,int* y){
	int z;
	z=*x;
	*x=*y;
	*y=z;
}

void set(int t,int* a,int* b){
	*a=(t/4)*3+(t)%3+1;
	*b=(t/4)*3+(t+1)%3+1;
}

//
int step3(int s6,int s7,int s4){
	int i,r,sa=0,sb,sc;
	for(i=1;i<8;i++)if(!used[i])break;
	if(i==8)fprintf(stderr,"!!\n");
	for(r=0;r<24;r++){
		if(r%4==0){sa++;set(sa,&sb,&sc);}
		if(graph[s4][6*i+sa-1]==SET && graph[s6][6*i+sb-1]==SET && graph[s7][6*i+sc-1]==SET)return 1;
		roll(&sb,&sc);
	}

	return 0;
}

int step2a(int da,int sa,int db,int sb,int dice,int *tc){
	int i,j,ta,tb;
	for(i=0;i<6;i++){
		if(graph[da*6+sa-1][dice*6+i]!=SET)continue;
		set(i+1,&ta,&tb);
		for(j=0;j<4;j++){
			if(graph[db*6+sb-1][dice*6+ta-1]==SET){
				*tc=tb;
				return 1;
			}
			roll(&ta,&tb);
		}
	}
	return 0;
}

int step2(int s5,int s3,int s2){
	int d5=s5/6,d2=s2/6,d3=s3/6;
	int t5=s5%6+1,t2=s2%6+1,t3=s3%6+1;
	int s5a,s5b,s2a,s2b,s3a,s3b;
	int s6c,s7c,s4c;
	int r,i,j,k;

	set(t5,&s5a,&s5b);
	set(t2,&s2a,&s2b);
	set(t3,&s3a,&s3b);

	for(r=0;r<64;r++){

		for(i=1;i<8;i++){
			if(used[i] || !step2a(d2,s2a,d5,s5b,i,&s6c))continue;
			used[i]=1;

			for(j=1;j<8;j++){
				if(used[j] || !step2a(d5,s5a,d3,s3b,j,&s7c))continue;
				used[j]=1;

				for(k=1;k<8;k++){
					if(used[k] || !step2a(d3,s3a,d2,s2b,k,&s4c))continue;
					used[k]=1;
					if(step3(i*6+s6c-1,j*6+s7c-1,k*6+s4c-1))return 1;
					used[k]=0;
				}

				used[j]=0;
			}

			used[i]=0;
		}


		roll(&s5a,&s5b);
		if(r%4==3)roll(&s2a,&s2b);
		if(r%16==15)roll(&s3a,&s3b);
	}

	return 0;
}

int step1(){
	int st=6,sa=2,sb=3,r,i,j,k;

	for(r=0;r<8;r++){

		for(i=6;i<48;i++){
			if(graph[st-1][i]!=SET)continue;
			used[i/6]=1;

			for(j=6;j<48;j++){
				if(used[j/6] || graph[sa-1][j]!=SET)continue;
				used[j/6]=1;

				for(k=6;k<48;k++){
					if(used[k/6] || graph[sb-1][k]!=SET)continue;
					used[k/6]=1;
					if(step2(i,j,k))return 1;
					used[k/6]=0;
				}

				used[j/6]=0;
			}

			used[i/6]=0;
		}

		roll(&sa,&sb);
		if(r==3){swap(&sa,&sb);st=1;}
	}

	return 0;
}

int main(){
	int i,j;
	used[0]=1;
	while(scanf("%s",cube[0])){
		SET++;
		if(cube[0][0]=='0')break;
		for(i=1;i<8;i++){scanf("%s",cube[i]);used[i]=0;}
		for(i=0;i<48;i++){
			for(j=0;j<48;j++){
				if(i/6!=j/6 && abs(cube[i/6][i%6]-cube[j/6][j%6])==GOOD)graph[i][j]=SET;
			}
		}
		if(step1())printf("YES\n"); else printf("NO\n");
	}
	return 0;
}