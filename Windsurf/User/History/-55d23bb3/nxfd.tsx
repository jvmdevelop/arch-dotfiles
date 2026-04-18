import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Info, CreditCard, User, CheckCircle } from "lucide-react"

export function DonateInstructions() {
  return (
    <Card className="border-border bg-card">
      <CardHeader>
        <CardTitle className="flex items-center gap-2 text-foreground">
          <Info className="h-5 w-5 text-primary" />
          Инструкции по донату
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        <div className="space-y-4">
          <div className="flex items-start gap-3">
            <div className="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-primary/10">
              <span className="text-xs font-medium text-primary">1</span>
            </div>
            <div className="space-y-1">
              <h3 className="font-medium text-foreground">Выберите тип поддержки</h3>
              <p className="text-sm text-muted-foreground">
                Выберите подходящий для вас вариант доната из доступных типов
              </p>
            </div>
          </div>

          <div className="flex items-start gap-3">
            <div className="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-primary/10">
              <span className="text-xs font-medium text-primary">2</span>
            </div>
            <div className="space-y-1">
              <h3 className="font-medium text-foreground">Введите никнейм</h3>
              <p className="text-sm text-muted-foreground">
                Укажите ваш игровой никнейм для идентификации платежа
              </p>
            </div>
          </div>

          <div className="flex items-start gap-3">
            <div className="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-primary/10">
              <span className="text-xs font-medium text-primary">3</span>
            </div>
            <div className="space-y-1">
              <h3 className="font-medium text-foreground">Перейдите к оплате</h3>
              <p className="text-sm text-muted-foreground">
                Нажмите кнопку оплаты и следуйте инструкциям платежной системы
              </p>
            </div>
          </div>

          <div className="flex items-start gap-3">
            <div className="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-primary/10">
              <span className="text-xs font-medium text-primary">4</span>
            </div>
            <div className="space-y-1">
              <h3 className="font-medium text-foreground">Получите вознаграждение</h3>
              <p className="text-sm text-muted-foreground">
                После успешной оплаты донат будет автоматически зачислен на ваш аккаунт
              </p>
            </div>
          </div>
        </div>

        <div className="rounded-lg bg-secondary/50 p-4">
          <h4 className="mb-2 font-medium text-foreground flex items-center gap-2">
            <CreditCard className="h-4 w-4" />
            Способы оплаты
          </h4>
          <p className="text-sm text-muted-foreground">
            Мы принимаем все основные способы оплаты: банковские карты, электронные кошельки и другие.
            Все платежи защищены и обрабатываются безопасно.
          </p>
        </div>

        <div className="rounded-lg bg-primary/5 border border-primary/20 p-4">
          <h4 className="mb-2 font-medium text-foreground flex items-center gap-2">
            <CheckCircle className="h-4 w-4 text-primary" />
            Важно знать
          </h4>
          <ul className="text-sm text-muted-foreground space-y-1">
            <li>• Донаты зачисляются автоматически в течение 5-15 минут</li>
            <li>• Убедитесь, что никнейм введен правильно</li>
            <li>• При проблемах с оплатой обратитесь в техподдержку</li>
            <li>• Все донаты являются необратимыми</li>
          </ul>
        </div>
      </CardContent>
    </Card>
  )
}
