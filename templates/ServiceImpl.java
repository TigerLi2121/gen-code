package {{ pkg_name }}.{{ module_name }}.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import {{ pkg_name }}.{{ module_name }}.dao.{{ class_name }}Dao;
import {{ pkg_name }}.{{ module_name }}.entity.{{ class_name }}Entity;
import {{ pkg_name }}.{{ module_name }}.service.{{ class_name }}Service;
import org.springframework.stereotype.Service;

/**
 * {{ comment }}
 *
 * @author {{ author }}
 * @date {{ date_time }}
 */
@Service("{{ class_name }}Service")
public class {{ class_name }}ServiceImpl extends ServiceImpl<{{ class_name }}Dao, {{ class_name }}Entity> implements {{ class_name }}Service {

}