#include <stdio.h>
#include <stdlib.h>

int main(){
    int q;
    scanf("%d\n",&q);
    
    int *a;
    int i=0;
    
    int com,x;
    while(q--){
        scanf("%d \n",&com);
        switch(com){
            case 0:
                scanf("%d\n",&x);
                if(i==0) a=(int*)malloc(sizeof(int));
                else a=(int*)realloc(a,sizeof(int)*(i+1));
                a[i++]=x;
                break;
            case 1:
                scanf("%d\n",&x);
                printf("%d\n",a[x]);
                break;
            case 2:
                a=(int*)realloc(a,sizeof(int)*(i--));
                break;
        }
    }
    
    free(a);
    
    return 0;
}
