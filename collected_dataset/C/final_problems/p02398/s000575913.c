#include <stdio.h>

int main(int argc, const char * argv[]) {
    // insert code here...
    int x=0,y=0,z=0,count=0;
    scanf("%d %d %d",&x,&y,&z);
    for(int i=x;i<=y;i++){
        if(z%i==0){
            count++;
        }
    }
    printf("%d\n",count);
    return 0;
}

