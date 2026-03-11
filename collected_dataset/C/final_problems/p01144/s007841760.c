#include <stdio.h>
#include <stdlib.h>

typedef struct{
    int d;
    int p;
}SYUKUBA;


int struct_sort( const void *a, const void *b ){
    return ((SYUKUBA*)b)->p - ((SYUKUBA*)a)->p;
}



int main(){
    int N,sum;
    long int M,count;
    int i;
    SYUKUBA n[10003];

    while(1){
        scanf("%d %ld",&N, &M);
        if(N == 0 && M == 0){
            break;
        }

        for(i = 0; i < N ; i++){
            scanf("%d %d",&n[i].d,&n[i].p);
            //printf("d = %d p = %d\n",n[i].d,n[i].p);
        }
        qsort(n,N,sizeof(SYUKUBA),struct_sort);
        /*
        for(i = 0; i < N ; i++){
            printf("s.d = %d s.p = %d\n",n[i].d,n[i].p);
        }
        */
       count = M;
       sum = 0;
       for(i = 0; i < N ; i++){
           if(n[i].d <= count){
               count -= n[i].d;
           }else if(n[i].d > count && count != 0){
               sum += (n[i].d - count) * n[i].p;
               count = 0;
           }else if(count == 0){
               sum += n[i].p * n[i].d;
           }

       }
       printf("%d\n",sum);

    }


    return 0;

}

