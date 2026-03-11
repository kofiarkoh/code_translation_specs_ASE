#include <stdio.h>
#include <stdlib.h>

typedef struct{
    char *e;
    int len;
}SET;

void output(SET s){
    int i;
    for(i=0;i<s.len;i++){
        if(i==0)printf("%d",s.e[i]);
        else printf(" %d",s.e[i]);
    }
    printf("\n");
    return;
}
void permutation(SET s,SET t,int num){
    if(num==t.len){
        output(t);
        return;
    }
    
    SET ss;
    ss.e=(char*)malloc(sizeof(char)*s.len);
    int i,j;
    for(i=0;i<s.len-num;i++){
        t.e[num]=s.e[i];
        
        for(j=0;j<s.len;j++)ss.e[j]=s.e[j];
        ss.len=s.len;
        for(j=i;j<s.len-1;j++)ss.e[j]=ss.e[j+1];
        
        permutation(ss,t,num+1);
    }
    
    return;
}

int main(void){
    int i;
    
    int n;
    scanf("%d\n",&n);
    
    SET s,t;
    s.len=n;
    s.e=(char*)malloc(sizeof(char)*n);
    for(i=0;i<n;i++)s.e[i]=i+1;
    t.len=n;
    t.e=(char*)malloc(sizeof(char)*n);
    
    permutation(s,t,0);
    

    return 0;
}

