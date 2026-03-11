#include<stdio.h>
#include<string.h>
void solve(int sl);
void inst(int c_num,int sw);
void delt(int c_num,int sw);
int fg;
int tlen[4]={5,7,5,6};
int len;
char S[1000000];
char T[1000000];

int main(void){
    int i,sw;
    scanf("%s",&S);
    len=strlen(S);
    fg=0;
    solve(0);
    if(fg==0){
        printf("NO\n");
    }else{
        printf("YES\n");
    }
    return 0;
}

void solve(int sl){
    int i;
    if(strcmp(S,T)==0){
        fg=1;
    }
    if(sl<len&&fg!=1){
        for(i=0;i<4;i++){
            inst(sl,i);
            solve(sl+tlen[i]);
            delt(sl,i);
        }
    }
}

void inst(int sl,int i){
    if(i==0){
        T[sl]='d';
        T[sl+1]='r';
        T[sl+2]='e';
        T[sl+3]='a';
        T[sl+4]='m';
    }else if(i==1){
        T[sl]='d';
        T[sl+1]='r';
        T[sl+2]='e';
        T[sl+3]='a';
        T[sl+4]='m';
        T[sl+5]='e';
        T[sl+6]='r';
    }else if(i==2){
        T[sl]='e';
        T[sl+1]='r';
        T[sl+2]='a';
        T[sl+3]='s';
        T[sl+4]='e';
    }else{
        T[sl]='e';
        T[sl+1]='r';
        T[sl+2]='a';
        T[sl+3]='s';
        T[sl+4]='e';
        T[sl+5]='r';
    }
    return;
}

void delt(int sl,int i){
    if(i==0){
        T[sl]=0;
        T[sl+1]=0;
        T[sl+2]=0;
        T[sl+3]=0;
        T[sl+4]=0;
    }else if(i==1){
        T[sl]=0;
        T[sl+1]=0;
        T[sl+2]=0;
        T[sl+3]=0;
        T[sl+4]=0;
        T[sl+5]=0;
        T[sl+6]=0;
    }else if(i==2){
        T[sl]=0;
        T[sl+1]=0;
        T[sl+2]=0;
        T[sl+3]=0;
        T[sl+4]=0;
    }else{
        T[sl]=0;
        T[sl+1]=0;
        T[sl+2]=0;
        T[sl+3]=0;
        T[sl+4]=0;
        T[sl+5]=0;
    }
    return;
}
