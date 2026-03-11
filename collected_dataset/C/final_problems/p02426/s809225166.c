/*
 * FileName:     bit_mask_fix
 * CreatedDate:  2020-07-27 20:25:33 +0900
 * LastModified: 2020-07-27 21:08:57 +0900
 */

#include <stdio.h>
#include <stdlib.h>

int main(void){
    long unsigned int bit = 0;
    int n;scanf("%d", &n);
    long unsigned int *box = malloc(n*sizeof(long unsigned int));
    for(int i=0; i<n; i++){
        int k;scanf("%d", &k);
        for(int j=0; j<k; j++){
            int b;scanf("%d", &b);
            box[i] += 1UL<<b;
        }
    }
    int q;scanf("%d", &q);
    for(int i=0; i<q; i++){
        int command, m;scanf("%d%d", &command, &m);
        if( command==0 ){
            printf("%lu\n", 1&bit>>m);
        }
        else if( command==1 ){
            bit |= box[m];
        }
        else if( command==2 ){
            bit &= ~box[m];
        }
        else if( command==3 ){
            bit ^= box[m];
        }
        else if( command==4 ){
            if( (bit&box[m])==box[m] ){
                printf("1\n");
            }
            else{
                printf("0\n");
            }
        }
        else if( command==5 ){
            if((bit&box[m])==0){
                printf("0\n");
            }
            else{
                printf("1\n");
            }
        }
        else if( command==6 ){
            printf("%d\n", !(bit&box[m]));
        }
        else if( command==7 ){
            printf("%d\n", __builtin_popcountll(bit&box[m]));
        }
        else{
            printf("%lu\n", bit&box[m]);
        }
    }
    free(box);
    return 0;
}

