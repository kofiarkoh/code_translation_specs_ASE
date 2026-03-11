#include <stdio.h>
#include <math.h>
#include <stdlib.h>
 
int door[20][20],sw[20][20],light[20],check[40000][20],prev[40000][20][2],q[500000][4];
int N,M,SET,GOAL,FR,TO,EXIT;
char stp[500000][40];

/*
Code - RoomNo --Steps --Prev
*/

int mark(int code,int now,int from){
	if(check[code][now]!=SET){
		check[code][now]=SET;
		prev[code][now][0]=q[FR][0];
		prev[code][now][1]=q[FR][1];
		q[TO][0]=code;
		q[TO][1]=now;
		q[TO][2]=q[FR][2]+1;
		q[TO][3]=from;
		TO++;
		if(now==N){
			EXIT=1;
			if(code==GOAL)return 1;
		}
	}
	return 0;
}

int search(int code,int now,int from){
	int i;

	for(i=0;i<N;i++){
		if( (code>>i & 1) && (door[now][i+1]==SET) ){
			if(mark(code,i+1,0))return 1;
		}
	}

	for(i=from-1;i<N;i++){
		if(now!=(i+1) && (sw[now][i+1])==SET ){
			if(mark(code^(1<<i),now,i+2))return 1;
		}
	}

	return 0;
}

void write(int code,int now,int pcode,int pre){
	if(now != pre){
		sprintf(stp[TO],"Move to room %d.\n",now);
	}
	else{
		int i;
		for(i=N-1;i>=0;i--){
			if( (code>>i) != (pcode>>i) ){
				sprintf(stp[TO],"Switch %s room %d.\n",(code>>i&1)?"on":"off",i+1);
				return;
			}
		}
	}
}
 
int main(){
    int i,j,k,x,y,code;
 
    while(scanf("%d %d",&N,&M)*(N+M)){
		SET++;

		GOAL=1<<N-1;

		for(i=0;i<M;i++){
			scanf("%d %d",&x,&y);
			door[x][y]=SET;
			door[y][x]=SET;
		}

		for(i=1;i<=N;i++){
			scanf("%d",&light[i]);
		}

		for(i=1;i<=N;i++){
			for(scanf("%d",&k);k>0;k--){
				scanf("%d",&x);
				sw[i][x]=SET;
				//if(i==x)fprintf(stderr,"??\n");
			}
		}

		code=0;
		
		for(i=N;i>0;i--){
			code=code*2+light[i];
		}

		q[0][0]=code;q[0][1]=1;q[0][2]=0;q[0][3]=1;
		FR=0;TO=1;EXIT=0;

		while(FR<TO){
			if(search(q[FR][0],q[FR][1],q[FR][3])){EXIT=2;break;}
			FR++;
		}

		if(EXIT==2){
			printf("You can go home in %d steps.\n",q[TO-1][2]);
			code=q[TO-1][0];x=q[TO-1][1];FR=q[TO-1][2];
			for(TO=FR-1;TO>=0;TO--){
				y=prev[code][x][1];
				i=prev[code][x][0];
				write(code,x,i,y);
				x=y;code=i;
			}
			for(TO=0;TO<FR;TO++){
				printf("%s",stp[TO]);
				//fprintf(stderr,"%s",stp[TO]);
			}

		}
		else if(EXIT==1)printf("You can not switch off all lights.\n");
		else printf("Help me!\n");
    }
  
    return 0;
}