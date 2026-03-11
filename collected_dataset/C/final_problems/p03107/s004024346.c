#include <stdio.h>
#include <stdlib.h>

//リストのノード（スタック内の各データ）の定義
typedef struct node{
    char elem;
    struct node *next;
} node;

//スタックにデータを追加
void push(node **first, char elem){
    //追加する分の領域を確保
    node *new = calloc(1, sizeof(node));
    if(new == NULL) exit(1);
    new->elem = elem;
    new->next = *first;
    *first = new;
}

//スタックからデータを取り出す
char pop(node **first){
    char elem;
    node *tmp = *first;
    if(tmp == NULL) exit(1);
    elem = tmp->elem;
    *first = tmp->next;
    //取り出したノードのメモリを解放
    free(tmp);
    return elem;
}

int main(void){
    char str;
    int ans = 0;
    //最初のノードの指す先
    node *first1 = NULL, *first2 = NULL;
    //入力を受け取りスタックに追加
    while(1){
        scanf("%c", &str);
        if(str == '0' || str == '1'){
            push(&first1, str);
        }else{
            break;
        }
    }

    //ループ開始
    while(first1 != NULL){
        //次のキューブがなければ終了
        if(first1->next == NULL){
            break;
        }
        //もし現在のキューブと次のキューブが違う色なら
        if(first1->elem != first1->next->elem){
            //現在のキューブと次のキューブを取り除き数を+2する
            pop(&first1);
            pop(&first1);
            ans += 2;
            //ひとつ前のキューブを戻す
            if(first2 != NULL){
                push(&first1, pop(&first2));
            }
        }else{
            //現在のキューブを保存する
            push(&first2, pop(&first1));
        }
    }

    //答えを出力
    printf("%d\n", ans);
    return 0;
}