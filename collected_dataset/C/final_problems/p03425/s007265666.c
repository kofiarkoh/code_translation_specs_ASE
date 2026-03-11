#include <stdio.h>

int main(void){
    int n;
    scanf("%d", &n);
    char name[11]; 
    int count[5] = {0};
    long ans = 0;
    for (int i = 0; i < n; i++){
        scanf("%s", name);
        switch(name[0]){
            case 'M':
            count[0] += 1;
            break;
            case 'A':
            count[1] += 1;
            break;
            case 'R':
            count[2] += 1;
            break;
            case 'C':
            count[3] += 1;
            break;
            case 'H':
            count[4] += 1;
            break;
            default:
            break;
        }
    }

    for(int i = 0; i < 3; i++){
        for(int j = i+1; j < 4; j++){
            for(int k = j+1; k < 5; k++){
                ans = ans + count[i] * count[j] * count[k];
            }
        }
    }
    printf("%ld\n", ans);
    return 0; 
}
