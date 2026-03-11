#include<stdio.h>

#define WORD_MAX 11

#define SENT_MAX 1000

#define FRAG_ON 1

#define FRAG_OFF 0

#define CONVERT_SMALL 0x60

#define LOOP 1

#define COUNT_RESET 0

#define LOOP_RESET 0

#define ARRAY_RESET 0

#define RETURN 0

void array_reset(unsigned char * array, unsigned short size);

unsigned short macth_word(unsigned char * word_1, unsigned char * word_2);

unsigned short s_scan_word(unsigned char * str, unsigned short length_max);

int main(){

        unsigned char ubW_D_word[WORD_MAX];                             /* 検索する単語 */

        unsigned char ubW_D_sent[SENT_MAX];                             /* 一致を照合する単語 */

        unsigned short usW_D_word_length;                               /* 検索する単語の文字数 */

        unsigned short usW_D_sent_length;                               /* 一致を照合する単語の文字数 */

        unsigned char ubW_D_sentence_end[] = {"END_OF_TEXT\n"};         /* 文章の終わりを示す文字列 */

        unsigned short usW_C_count = COUNT_RESET;                       /* 一致した単語のカウンタ */

        unsigned short usW_C_i;                                         /* ループ変数 */

        

        /* 配列の初期化 */

        array_reset(ubW_D_word, WORD_MAX);

        array_reset(ubW_D_sent, SENT_MAX);

        

        /* 検索する単語の入力 */

        usW_D_word_length = s_scan_word(ubW_D_word, WORD_MAX);

        

        /* 文章の入力、照合 */

        while(LOOP){

                /* 単語を入力 */

                usW_D_sent_length = s_scan_word(ubW_D_sent, SENT_MAX);

                

                /* 入力終了処理 */

                if(macth_word(ubW_D_sentence_end, ubW_D_sent) == FRAG_ON){

                        break;

                }

                

                /* 大文字アルファベットを小文字に変換 */

                for(usW_C_i = LOOP_RESET;usW_C_i < usW_D_sent_length;usW_C_i++){

                        ubW_D_sent[usW_C_i] |= CONVERT_SMALL;

                }

                

                /* 検索する/される単語の文字数が同じ場合比較 */

                if(usW_D_word_length == usW_D_sent_length){

                        /* 二つの単語が一致していればカウンタをインクリメント */

                        usW_C_count += macth_word(ubW_D_word, ubW_D_sent);

                }

                

                /* 一致を照合する単語の初期化 */

                array_reset(ubW_D_sent, usW_D_sent_length);

        }

        

        /* 結果の出力 */

        printf("%d\n", usW_C_count);

        return RETURN;

}

/* 関数名：配列初期化 */

/* 概要：渡されたアドレスを先頭とする配列の値を初期化する */

/* 引数：初期化する配列の先頭アドレス、その配列の大きさ */

/* 戻り値：なし */

void array_reset(unsigned char * array, unsigned short size)

{

        unsigned short usW_C_i;         /* ループ変数 */

        /* 配列の初期化 */

        for(usW_C_i = COUNT_RESET;usW_C_i < size;usW_C_i++){

                *(array + usW_C_i) = ARRAY_RESET;

        }

}

/* 関数名：文字列入力 */

/* 概要：文字列を入力する。入力された文字列の長さを返す */

/* 引数：文字列を格納する配列の先頭アドレス、その配列の大きさ */

/* 戻り値：入力された文字列の長さ */

unsigned short s_scan_word(unsigned char * str, unsigned short length_max)

{

        unsigned short usW_C_count_text = COUNT_RESET;          /* 文字列長カウンタ */

        unsigned short usW_C_i;                                 /* ループ変数 */

        /* 文字列の入力 */

        for(usW_C_i = LOOP_RESET;usW_C_i < length_max;usW_C_i++){

                scanf("%c", str + usW_C_i);

                /* 入力終了の処理 */

                if((*(str + usW_C_i) == ' ') || (*(str + usW_C_i) == '\n')){

                        break;

                }

                /* 文字数をカウント */

                usW_C_count_text++;

        }

        /* 文字列長を返す */

        return usW_C_count_text;

}

/* 関数名：文字列照合 */

/* 概要：渡された二つの文字列が一致しているか調べる */

/* 引数：文字列１、文字列２ */

/* 戻り値：照合の結果一致していれば１、一致していなければ０を返す */

unsigned short macth_word(unsigned char * word_1, unsigned char * word_2)

{

        unsigned int usW_F_frg = FRAG_OFF;        /* 一致フラグ */

        unsigned int usW_C_i = LOOP_RESET;        /* ループ変数 */

        

        /* 文字列の照合 */

        while(LOOP){

                /* 文字列の終端であればフラグをオンにする */

                if(*(word_1 + usW_C_i) == '\n'){

                        usW_F_frg = FRAG_ON;

                }

                /* 文字が異なればループを抜ける */

                if(*(word_1 + usW_C_i) != *(word_2 + usW_C_i)){

                        break;

                }

                /* ループ変数をインクリメント */

                usW_C_i++;

        }

        

        return usW_F_frg;

}
