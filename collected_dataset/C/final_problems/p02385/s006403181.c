#include <stdio.h>
struct dice{
    int d[6], tmp[6];
};
struct dice dice1;
struct dice dice2;
void init1(int n[6]){
    int i;
    for (i=0;i<6;i++){
        dice1.d[i]=n[i];
    }
}
void init2(int n[6]){
    int i;
    for (i=0;i<6;i++){
        dice2.d[i]=n[i];
    }
}
void throwdice(char c){
    int i;
    for(i=0;i<6;i++){
        dice1.tmp[i]=dice1.d[i];
    }
    if(c=='N'){
        dice1.d[0]=dice1.tmp[1];
        dice1.d[1]=dice1.tmp[5];
        dice1.d[2]=dice1.tmp[2];
        dice1.d[3]=dice1.tmp[3];
        dice1.d[4]=dice1.tmp[0];
        dice1.d[5]=dice1.tmp[4];
    }
    else if(c=='E'){
        dice1.d[0]=dice1.tmp[3];
        dice1.d[1]=dice1.tmp[1];
        dice1.d[2]=dice1.tmp[0];
        dice1.d[3]=dice1.tmp[5];
        dice1.d[4]=dice1.tmp[4];
        dice1.d[5]=dice1.tmp[2];
    }
    else if(c=='W'){
        dice1.d[0]=dice1.tmp[2];
        dice1.d[1]=dice1.tmp[1];
        dice1.d[2]=dice1.tmp[5];
        dice1.d[3]=dice1.tmp[0];
        dice1.d[4]=dice1.tmp[4];
        dice1.d[5]=dice1.tmp[3];
    }
    else if(c=='S'){
        dice1.d[0]=dice1.tmp[4];
        dice1.d[1]=dice1.tmp[0];
        dice1.d[2]=dice1.tmp[2];
        dice1.d[3]=dice1.tmp[3];
        dice1.d[4]=dice1.tmp[5];
        dice1.d[5]=dice1.tmp[1];
    }
    else if(c=='R'){
        dice1.d[0]=dice1.tmp[0];
        dice1.d[1]=dice1.tmp[3];
        dice1.d[2]=dice1.tmp[1];
        dice1.d[3]=dice1.tmp[4];
        dice1.d[4]=dice1.tmp[2];
        dice1.d[5]=dice1.tmp[5];
    }
}
int compare(){
    int i;
    for(i=0;i<6;i++){
        if(dice1.d[i]!=dice2.d[i]){
            return 0;
        }
    }
    return 1;
}
int main()
{
    int n[6];
    int i,j;
    int ok=0;
    for(i=0;i<6;i++){
        scanf("%d",&n[i]);
    }
    init1(n);
    for(i=0;i<6;i++){
        scanf("%d",&n[i]);
    }
    init2(n);
    for(i=0;i<4;i++){
        for(j=0;j<4;j++){
            throwdice('N');
            throwdice('E');
            throwdice('S');
            if(compare()) ok=1;
        }
        throwdice('N');    
    }
    throwdice('E');
    for(i=0;i<2;i++){
        for(j=0;j<4;j++){
            throwdice('N');
            throwdice('E');
            throwdice('S');
            if(compare()) ok=1;
        }
        throwdice('W');    
        throwdice('W');    
    }
    if(ok) puts("Yes");
    else puts("No");
    return 0;
}

