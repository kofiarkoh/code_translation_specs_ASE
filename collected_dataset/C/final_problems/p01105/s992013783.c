/************************************************************
 ICPC 2017 国内予選問題E 論理式圧縮機
 http://icpc.iisf.or.jp/past-icpc/domestic2017/contest/all_ja.html#section_E
  Filename:  pe.c
  Compile:   cc -Wall pe.c
  Execution: ./a.out < E0 > E0.result; diff E0.ans E0.result 等
  Algorithm: 4変数の論理関数は 2^2^4 = 2^16 = 65,536 しかない。
             長さ16以下の論理式を全て生成し、各論理関数に対する
             論理式の長さの最小値を覚えておく。
************************************************************/

#include <stdio.h>
#define SIZE 1000000   // 長さ16以下の論理式の個数（取り敢えずこの程度に）
#include <string.h>
#define N_EXP (1 << 16) // 4変数論理関数の個数
#define MAXLEN 16       // 論理式の長さの最大値
int num;                // 論理式の個数
char expression[SIZE][MAXLEN+1];        // 作成した論理式を記憶する論理式表
typedef struct {
    int len;            // 論理式の長さの最小値
    int pos;            // 論理式の文字列の格納位置(expression配列)
} info_t;
info_t info[N_EXP];     // 各論理関数の情報(論理関数表)

// 論理関数に対する論理式情報を初期化する関数
void init_info(){
    int i;
    for(i=0; i<N_EXP; i++){     // 各論理関数に対して
        info[i].len = MAXLEN+1; // 論理式の長さを最大値＋１(17)とする
        info[i].pos = -1;       // この論理関数に対する論理式は見つかっていない
    }
}

// 論理式 str の全入力に対する値(16bit)を求める
int eval_pos;                   // 現在処理中の論理式の位置
unsigned short eval(char *str){
    char token;                 // これから処理する文字
    unsigned short v1, v2;                 // 引数の値
    token = str[eval_pos++];    // １文字取得
    switch(token){
    case '0':                   // 定数 0
        return 0;
    case '1':                   // 定数 1
        return 0xFFFF;
    case 'a':                   // 論理変数 a (最上位ビット)
        return 0xFF00;
    case 'b':                   // 論理変数 b (上から２番目のビット)
        return 0xF0F0;
    case 'c':                   // 論理変数 c (下から２番目のビット)
        return 0xCCCC;
    case 'd':                   // 論理変数 d (最下位ビット)
        return 0xAAAA;
    case '-':                   // 論理否定
        v1 = eval(str);         // 引数の値を求める
        return ~v1;
    case '(':                   // 左括弧開始
        v1 = eval(str);                 // 第一引数の値を求める
        if(str[eval_pos++]=='^'){       // EXOR演算
            v2 = eval(str);             // 第二引数の値を求める
            if(str[eval_pos++] != ')')
                fprintf(stderr, "missing ) for ^\n");   // 文法誤り
            return v1 ^ v2;             // EXOR演算を適用
        }
        else {                          // 論理積演算
            v2 = eval(str);             // 第二引数の値を求める
            if(str[eval_pos++] != ')')
                fprintf(stderr, "missing ) for ^\n");   // 文法誤り            
            return v1 & v2;             // 論理積演算を適用
        }
    default:                    // ここには来ないハズ
        break;
    }
    return 0;                   // ここにも来ないハズ
}

// 論理式に対する論理関数の ID を求める (真理値ベクトル）
unsigned short func_id(char *str){
    eval_pos = 0;
    return eval(str);
}

// 論理式を論理関数表と論理式表に登録
void insert(char *str){
    int len;
    unsigned short id;
    len = strlen(str);          // 論理式の長さ
    if(len > MAXLEN) return;    // MAXLEN(16)より長い場合は登録しない
    id = func_id(str);          // 論理式が表す論理関数のIDを求める
    if(len < info[id].len){     // この論理式の長さの方が短ければ
        info[id].len = len;     // 長さを更新
        if(info[id].pos >= 0)   // 過去に論理式が登録されていれば
            expression[info[id].pos][0] = 0;    // その論理式を無効にする
        info[id].pos = num;     // この論理式の場所を登録
        strncpy(expression[num++], str, MAXLEN+1);      // 論理式表に登録
//        printf("%d: %s (id=%hd len=%d)\n", num-1, str, id, len);
    }
}

// 論理式表、論理関数表を作成
#define BUFF_SIZE 50
void create_table(){
    char buff[BUFF_SIZE];
    int i, j;
    int newstart, nextnew;
    num = 0;
    init_info();                // 論理関数表を初期化
    insert("0"); insert("1");   // 定数を登録
    insert("a"); insert("b"); insert("c"); insert("d"); // 論理変数を登録
    newstart = 0;               // 今回の新規登録開始位置
    nextnew = num;              // 次の新規登録開始位置
    while(1){
        // - を適用
        for(i=newstart; i<nextnew; i++){                // 新しい項に対し
            if(expression[i][0] == '-') continue;       // -の連続適用はしない
            if(expression[i][0] == 0) continue; // より短い表現があるのでskip
            strncpy(buff, "-", BUFF_SIZE);      // 論理式を作成
            strncat(buff, expression[i],BUFF_SIZE);
            insert(buff);       // 登録
        }
        // * or ^ を適用
        for(i=newstart; i<nextnew; i++){        // 最初の引数は新しい項とする
            if(expression[i][0] == 0) continue; // より短い表現があるので skip
            for(j=0; j<nextnew; j++){           // 次の項は古い項でもOK
                if(expression[j][0] == 0) continue;  // 短い表現があるのでskip
                // EXOR 論理式を作成して登録
                strncpy(buff, "(", BUFF_SIZE);
                strncat(buff,expression[i],BUFF_SIZE);
                strncat(buff,"^",BUFF_SIZE);
                strncat(buff,expression[j],BUFF_SIZE);
                strncat(buff, ")",BUFF_SIZE);
                insert(buff);
                // AND 論理式を作成して登録
                strncpy(buff, "(", BUFF_SIZE);
                strncat(buff,expression[i],BUFF_SIZE);
                strncat(buff,"*",BUFF_SIZE);
                strncat(buff,expression[j],BUFF_SIZE);
                strncat(buff, ")",BUFF_SIZE);
                insert(buff);
            }
        }
        if(nextnew == num) break;       // 新しい論理式は登録されなかった
        newstart = nextnew;     // 新規登録位置を更新
        nextnew = num;          // 次の新規登録位置も更新
    }
}    

int main(){
    unsigned short id;
    int i;
    create_table();             // 論理式表、論理関数表を作成
//    printf("num=%d\n", num);
    char buff[MAXLEN+2];
    while(1){
        fgets(buff, MAXLEN+2, stdin);   // データセットを入力
        if(buff[0] == '.') break;       // 最後は「.」のみ
        for(i=0; i<MAXLEN; i++){        // 改行文字をNULL文字にする
            if(buff[i] == '\n'){
                buff[i] = 0;
                break;
            }
        }
        id = func_id(buff);             // 入力された論理式の論理関数IDを求める
//        printf("%d %hd\n", info[id].len, id);
        // その論理関数を表す論理式の長さの最小値をプリント
        printf("%d\n", info[id].len);
    }
    return 0;
}

