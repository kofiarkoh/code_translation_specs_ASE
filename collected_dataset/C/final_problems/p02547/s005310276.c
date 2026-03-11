#include <stdio.h>
#include <string.h>

int main(int argc, const char * argv[]) {
    // insert code here...
    int n = 0, i, j, cnt=0, save=0, connect=0;//connect=0初回  ３連続でない 7連続である
    int s[6][2];
   
    scanf("%d",&n);
    
    for (i = 0; i < n; i++) {
        for (j = 0; j < 2; j++) {
            scanf("%d", &s[i][j]);
        }
    }
    
    
//    printf("%d", n);
//    printf("\n");
//    for (i = 0; i < n; i++) {
//        for (j = 0; j < 2; j++) {
//            printf("%d ", s[i][j]);
//        }
//        printf("\n");
//    }
    for(i = 0; i < n; i++){
        if(s[i][0]==s[i][1]){
            if(connect==0||connect==7){
                cnt++;
            }else{
                cnt=1;
            }
            connect=7;
        }else{
           
            if(save<cnt){
                save = cnt;
            }
        
            connect=3;
        }
    }
    if(save>cnt){
        cnt=save;
    }
  
    if(cnt>=3){
        printf("Yes\n");
    }else{
        printf("No\n");
    }
    return 0;
}
