#include<stdio.h>

int main(void){

int i,j,k;//制御
char str[15];
int maru=0,batu=0;

scanf("%s",str);

for(i=0;i<15;i++){
    if(str[i]=='o'){
        maru++;
    }
    else if(str[i]=='x'){
        batu++;
    }
}
if(batu<=7){
    printf("YES");
}
else if(maru>=8){
    printf("YES");
}
else
{
    printf("NO");
}


return 0;

}
