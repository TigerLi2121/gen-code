package {{ pkg_name }}.{{ module_name }}.controller;

import com.baomidou.mybatisplus.core.metadata.IPage;
import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.ximalaya.eros.web.anno.RestPathController;
import {{ pkg_name }}.{{ module_name }}.entity.{{ class_name }}Entity;
import {{ pkg_name }}.{{ module_name }}.service.{{ class_name }}Service;
import com.ximalayaos.common.util.R;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.*;

import javax.validation.constraints.NotNull;
import java.util.Arrays;
import java.util.List;

/**
 * {{ comment }}
 *
 * @author {{ author }}
 * @date {{ date_time }}
 */
@Slf4j
@Validated
@RequiredArgsConstructor
@RestPathController("/api/{{ table_name }}")
public class {{ class_name }}Controller {

    @Autowired
    private {{ class_name }}Service {{ class_name_fl }}Service;

    /**
     * 分页
     */
    @GetMapping
    public R.Page<{{ class_name }}Entity> page(@NotNull(message = "page is null") Integer page,
                                                @NotNull(message = "limit is null") Integer limit) {
        IPage<{{ class_name }}Entity> iPage = {{ class_name_fl }}Service.page(new Page<>(page, limit));
        return R.ok(iPage);
    }

    /**
     * 新增or修改
     */
    @SysLog("保存修改{{ comment }}")
    @PostMapping
    public R sou(@RequestBody {{ class_name }}Entity entity) {
        {{ class_name_fl }}Service.saveOrUpdate(entity);
        return R.ok();
    }

    /**
     * 删除
     */
    @SysLog("删除{{ comment }}")
    @DeleteMapping
    public R del(@RequestBody Long[] ids) {
        {{ class_name_fl }}Service.removeByIds(Arrays.asList(ids));
        return R.ok();
    }
}
