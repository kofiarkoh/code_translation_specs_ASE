//C
#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>

void revstr(char* str);
void ternary(int i, char *t);
int decimai(char *j);
void salsa(int *P,int max);
void rumba(int *P,int max);

int main(void){
    int N,max,l,k=0;
    char T[200000];
    int P[531441];
    
    scanf("%d",&N);
    scanf("%s",T);
    
    max=(int)pow(3,N);
    for(k=0;k<max;k++)
        P[k]=k;
    k=0;
    while(T[k]!='\0'){
        if(T[k]=='S')
            salsa(P,max);
        else if(T[k]=='R')
            rumba(P,max);
        k++;
    }
    for(k=0;k<max;k++){
        for(l=0;l<max&&P[l]!=k;l++);
        if(k!=max-1)printf("%d ",l);
    }
    printf("%d\n",l);
    return 0;
}

void revstr(char* str){
	int size = strlen(str);
	int i,j;
	char tmp = {0};
	
	for(i = 0, j = size - 1; i < size / 2; i++, j--){
		tmp = str[i];
		str[i] = str[j];
		str[j] = tmp;
	}
	
	return;	
}

void ternary(int i, char *t){
    int count=0;
    while(i/3>=0){
        char tmp[2];
        sprintf(tmp, "%d", i%3);
        if(i%3==1)
            t[count++]='2';
        else if(i%3==2)
            t[count++]='1';
        else
            t[count++]=tmp[0];
            
        if(i/3==0)
            break;
            
        i/=3;
    }
    t[count]='\0';
    revstr(t);
}

int decimai(char *j){
    int len=strlen(j);
    int count;
    int d=0;
    for(count=0;count<len;count++){
        char tmp[2];
        tmp[0]=j[count];
        tmp[1]='\0';
        d+=(int)pow(3,len-count-1)*atoi(tmp);
    }
    
    return d;
}
void salsa(int *P,int max){
    int i,tmp[531441];
    char t[20];
    for(i=0;i<max;i++){
        ternary(i,t);
        tmp[decimai(t)]=P[i];
    }
    for(i=0;i<max;i++)
        P[i]=tmp[i];
}
void rumba(int *P,int max){
    int i,tmp[531441];
    for(i=0;i<max-1;i++){
        tmp[i+1]=P[i];
    }
    tmp[0]=P[max-1];
    for(i=0;i<max;i++)
        P[i]=tmp[i];
}
