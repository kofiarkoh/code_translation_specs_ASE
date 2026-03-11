/* ex 7_3
   TakayukiY*/

#include<stdio.h>
#include<stdlib.h>

int main(void){
    //indexと行列の要素数と空行チェック用の配列のポインタの定義
    int i,j,row,column,*chkrow,*chkcol;
    //aijの格納配列の定義
    char **ptr;

    //要素数を代入して各行のメモリ確保、エラーチェック
    scanf("%d%d\n",&row,&column);
    ptr=(char**)calloc(row,sizeof(char*));
    if(ptr==NULL) exit(1);

    for(i=0;i<row;i++){
        //各列のメモリ確保、エラーチェック
        ptr[i]=(char*)calloc(column,sizeof(char));
        if(ptr==NULL) exit(1);
        
        //各要素を代入
        for(j=0;j<column;j++){
            scanf("%c",&ptr[i][j]);
        }
        //末尾の改行をスキップ
        getchar();
    }

    //空行チェック用の配列のメモリ確保
    chkrow=(int*)calloc(row,sizeof(int));
    if(chkrow==NULL) exit(1);
    chkcol=(int*)calloc(column,sizeof(int));
    if(chkcol==NULL) exit(1);

    //行列の各要素のいずれかが空白でないならチェック用配列に1を代入
    for(i=0;i<row;i++){
        for(j=0;j<column;j++){
            if(ptr[i][j]=='#'){
                *(chkrow+i)=1;
                *(chkcol+j)=1;
            }
        }
    }

    //空白でない行・列に対して出力→圧縮
    for(i=0;i<row;i++){
        if(chkrow[i]==1){
            for(j=0;j<column;j++){
                if(chkcol[j]==1){
                    printf("%c",ptr[i][j]);
                }
            }
            printf("\n");
        }
    }

    //メモリの解放
    for(i=0;i<row;i++){
        free(ptr[i]);
    }
    free(ptr);
    free(chkrow);
    free(chkcol);

    return 0;
}
