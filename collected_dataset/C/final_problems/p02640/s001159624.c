#include<stdio.h>
int main(){
    int x,y,i,j;
    int flag = 0;
    scanf("%d %d",&x,&y);
    
    for(i = 0; i <= x; i++){
        j = x-i;
        if((2*i) + (4*j) == y) flag = 1;
    }
    if(flag == 1) printf("Yes\n");
    else printf("No\n");
    return 0;
}
