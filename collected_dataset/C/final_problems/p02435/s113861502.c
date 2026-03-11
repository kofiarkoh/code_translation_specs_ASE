#include <stdio.h>
#include <stdlib.h>

typedef struct _vec{
    int *data;
    int cnt;
}VEC;

int main(){
    int i;
    int n,q;
    scanf("%d %d\n",&n,&q);
    
    VEC v[n];
    for(i=0;i<n;i++){
        v[i].cnt=0;
    }
    
    int com,t,x;
    while(q--){
        scanf("%d %d \n",&com,&t);
        switch(com){
            case 0:
                //pushback
                scanf("%d\n",&x);
                if(v[t].cnt==0){
                    v[t].data=(int*)malloc(sizeof(int));
                }else{
                    v[t].data=(int*)realloc(v[t].data,sizeof(int)*(v[t].cnt+1));
                }
                v[t].data[v[t].cnt++]=x;
                break;
            case 1:
                //dump
                if(v[t].cnt!=0)printf("%d\n",v[t].data[v[t].cnt-1]);
                break;
            case 2:
                //clear
                if(v[t].cnt!=0){
                    v[t].data=(int*)realloc(v[t].data,sizeof(int)*(--v[t].cnt));
                }
                break;
        }
    }
    
    for(i=0;i<n;i++){
        if(v[t].cnt!=0){
            free(v[t].data);
            v[t].cnt=0;
        }
    }
    
    return 0;
}
