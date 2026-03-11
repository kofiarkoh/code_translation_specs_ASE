//ALDS1_3_D

#include <stdio.h>
#include <string.h>

#define STR_MAX_LEN 20000

int main(void)
{
    char            buf[STR_MAX_LEN];
    unsigned short  length[STR_MAX_LEN] = {}, depth;
    unsigned short  ii, jj, kk, field_len, floos[STR_MAX_LEN / 2] = {};
    unsigned short  valley = 1, hill[STR_MAX_LEN / 2] = {}; 
    unsigned int    total = 0;
    
    fgets(buf, sizeof(buf), stdin);
    
    field_len = strlen(buf);
    depth = 0;

    for(ii = 0; ii < field_len; ii++){
        switch(buf[ii]){
            case '\\':
                for(jj = 0; jj <= depth; jj++){
                    length[jj]++;
                }
                if(valley == 1){
                    hill[++hill[0]] = depth;
                    floos[0]++;
                    valley = 0;
                }
                depth++;
                break;
            case  '_':
                for(jj = 0; jj <= depth; jj++){
                    length[jj]++;
                }
                break;
            case  '/':
                for(jj = 0; jj <= depth; jj++){
                    length[jj]++;
                }
                if(0 < depth){
                    floos[floos[0]] += length[depth];
                    length[depth] = 0;
                    if(depth == hill[hill[0]]){
                        floos[0]--;
                        floos[floos[0]] += floos[floos[0] + 1];
                        floos[floos[0] + 1] = 0;
                        hill[hill[0]--] = 0;
                    }
                    depth--;
                    valley = 1;
                }
                break;
            default:
                break;
        }
    }
    
    if(valley == 0){
        floos[0]--;
        floos[floos[0]] += floos[floos[0] + 1];
        floos[floos[0] + 1] = 0;
    }

    for(ii = 1; ii <= floos[0]; ii++){
        total += floos[ii];
    }
    printf("%d\n", total);
    printf("%d", floos[0]);
    for(ii = 1; ii <= floos[0]; ii++){
        printf(" %d", floos[ii]);
    }
}
