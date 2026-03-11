#include<stdio.h>
#include<stdlib.h>
int main(){
    char **c;
    char **cc;
    int h,w;
    scanf("%d %d",&h,&w);
    c=calloc(h,sizeof(char*));
    cc=calloc(2*h,sizeof(char*));
    for(int i=0;i<h;i++){
        c[i]=calloc(w,sizeof(char));
    }
    for(int i=0;i<2*h;i++){
        cc[i]=calloc(w,sizeof(char));
    }
    for(int i=0;i<2*h;i++){
        for(int j=0;j<w;j++){
            cc[i][j]='.';
        }
    }
    for(int i=0;i<h;i++){
        scanf("%s",c[i]);
    }
    for(int j=0;j<w;j++){
        int k=0;
        for(int i=0;i<h;i++){
            for(int l=0;l<2;l++){
                if(c[i][j]=='.') cc[k][j]=cc[k+1][j]='.';
                if(c[i][j]=='*') cc[k][j]=cc[k+1][j]='*';
            }
            k+=2;
        }
    }
    for(int i=0;i<2*h;i++){
        for(int j=0;j<w;j++){
            printf("%c",cc[i][j]);
        }
        printf("\n");
    }

    for(int i=0;i<h;i++){
        scanf("%s",c[i]);
    }
    printf("\n");
    return 0;
}