
//c.......

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
int b[200001];
int main(){
    int n,m;
    scanf("%d%d",&n,&m);
    int a,aa;
    for (int i=0;i<m; i++) {
        scanf("%d%d",&a,&aa);
        b[a]=aa;
    }
    int i = 0;
    for(i=1;i<200001;i++){
        if(b[i]==n) break;
    }
    if(b[1]==i) printf("POSSIBLE");
    else printf("IMPOSSIBLE");
    return 0;
}
