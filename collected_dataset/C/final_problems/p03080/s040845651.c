#include <stdlib.h>
#include <stdio.h>

int main(){
    int N;
    scanf("%d", &N);

    char s[105];
    int i;
    int num_red = 0;
    int num_blue = 0;
    // for(i=0; i<N; i++){
    //     s[i] = getchar();
    //     if(s[i] == "R"){
    //         num_red ++;
    //     }else if(s[i] == "B"){
    //         num_blue ++;
    //     }
    // }

    for(i=0; i<N; i++){
        char temp = getchar();
        if(temp == 'R'){
            num_red ++;
        }else if(temp == 'B'){
            num_blue ++;
        }
    }

    if(num_red > num_blue){
        printf("Yes\n");
    }else{
        printf("No\n");
    }


    return 0;
}