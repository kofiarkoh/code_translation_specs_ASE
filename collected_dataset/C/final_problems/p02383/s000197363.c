#include<stdio.h>
#include<string.h>

	int dice[6];
	int i,t;
	
void nmov(){
	int buf;
	 buf = dice[0];
	dice[0] = dice[1];
	dice[1] = dice[5];
	dice[5] = dice[4];
	dice[4] = buf;
}
void emov(){
	int buf;
	buf = dice[0];
	dice[0] = dice[3];
	dice[3] = dice[5];
	dice[5] = dice[2];
	dice[2] = buf;
}
void wmov(){
	int buf;
	buf = dice[0];
	dice[0] = dice[2];
	dice[2] = dice[5];
	dice[5] = dice[3];
	dice[3] = buf;
}
void smov(){
int buf;
	buf = dice[0];
	dice[0] = dice[4];
	dice[4] = dice[5];
	dice[5] = dice[1];
	dice[1] = buf;
}

int main(){
	for(i=0;i<6;i++){
	scanf("%d",&dice[i]);
	}
	char c[105];
	scanf("%s",c);
	for(i=0; i<strlen(c); i++){
		if(c[i]=='N'){nmov();}
		else if(c[i]=='E'){emov();}
		else if(c[i]=='W'){wmov();}
		else if(c[i]=='S'){smov();}
	}
	printf("%d\n",dice[0]);
         
    return 0;
}

